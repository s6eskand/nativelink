// Copyright 2024 The NativeLink Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::pin::Pin;
use core::task::{Context as TaskContext, Poll};

use futures::Future;
use hyper::rt::Executor;
use hyper_util::rt::tokio::TokioExecutor;
use opentelemetry::context::{Context, FutureExt};
use tokio::task::{JoinError, JoinHandle, spawn_blocking};
pub use tracing::error_span as __error_span;
use tracing::{Instrument, Span};

pub fn __spawn_with_span_and_context<F, T>(f: F, span: Span, ctx: Option<Context>) -> JoinHandle<T>
where
    T: Send + 'static,
    F: Future<Output = T> + Send + 'static,
{
    let future = f.instrument(span);
    let future = if let Some(ctx) = ctx {
        future.with_context(ctx)
    } else {
        future.with_current_context()
    };

    #[expect(clippy::disallowed_methods, reason = "purpose of the method")]
    tokio::spawn(future)
}

pub fn __spawn_with_span<F, T>(f: F, span: Span) -> JoinHandle<T>
where
    T: Send + 'static,
    F: Future<Output = T> + Send + 'static,
{
    let current_ctx = Context::current();
    __spawn_with_span_and_context(f, span, Some(current_ctx))
}

pub fn __spawn_blocking<F, T>(f: F, span: Span) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    #[expect(clippy::disallowed_methods, reason = "purpose of the method")]
    spawn_blocking(move || span.in_scope(f))
}

#[macro_export]
macro_rules! background_spawn {
    ($name:expr, $fut:expr) => {{
        $crate::task::__spawn_with_span($fut, $crate::task::__error_span!($name))
    }};
    ($name:expr, $fut:expr, $($fields:tt)*) => {{
        $crate::task::__spawn_with_span($fut, $crate::task::__error_span!($name, $($fields)*))
    }};
    (name: $name:expr, fut: $fut:expr, target: $target:expr, $($fields:tt)*) => {{
        $crate::task::__spawn_with_span($fut, $crate::task::__error_span!(target: $target, $name, $($fields)*))
    }};
    (span: $span:expr, ctx: $ctx:expr, fut: $fut:expr) => {{
        $crate::task::__spawn_with_span_and_context($fut, $span, $ctx)
    }};
}

#[macro_export]
macro_rules! spawn {
    ($name:expr, $fut:expr) => {{
        $crate::task::JoinHandleDropGuard::new($crate::background_spawn!($name, $fut))
    }};
    ($name:expr, $fut:expr, $($fields:tt)*) => {{
        $crate::task::JoinHandleDropGuard::new($crate::background_spawn!($name, $fut, $($fields)*))
    }};
    (name: $name:expr, fut: $fut:expr, target: $target:expr, $($fields:tt)*) => {{
        $crate::task::JoinHandleDropGuard::new($crate::background_spawn!($name, $fut, target: $target, $($fields)*))
    }};
}

#[macro_export]
macro_rules! spawn_blocking {
    ($name:expr, $fut:expr) => {{
        $crate::task::JoinHandleDropGuard::new($crate::task::__spawn_blocking($fut, $crate::task::__error_span!($name)))
    }};
    ($name:expr, $fut:expr, $($fields:tt)*) => {{
        $crate::task::JoinHandleDropGuard::new($crate::task::__spawn_blocking($fut, $crate::task::__error_span!($name, $($fields)*)))
    }};
    ($name:expr, $fut:expr, target: $target:expr) => {{
        $crate::task::JoinHandleDropGuard::new($crate::task::__spawn_blocking($fut, $crate::task::__error_span!(target: $target, $name)))
    }};
    ($name:expr, $fut:expr, target: $target:expr, $($fields:tt)*) => {{
        $crate::task::JoinHandleDropGuard::new($crate::task::__spawn_blocking($fut, $crate::task::__error_span!(target: $target, $name, $($fields)*)))
    }};
}

/// Simple wrapper that will abort a future that is running in another spawn in the
/// event that this handle gets dropped.
#[derive(Debug)]
#[must_use]
pub struct JoinHandleDropGuard<T> {
    inner: JoinHandle<T>,
}

impl<T> JoinHandleDropGuard<T> {
    pub const fn new(inner: JoinHandle<T>) -> Self {
        Self { inner }
    }
}

impl<T> Future for JoinHandleDropGuard<T> {
    type Output = Result<T, JoinError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx)
    }
}

impl<T> Drop for JoinHandleDropGuard<T> {
    fn drop(&mut self) {
        self.inner.abort();
    }
}

#[derive(Debug, Clone)]
pub struct TaskExecutor(TokioExecutor);

impl TaskExecutor {
    pub fn new() -> Self {
        Self(TokioExecutor::new())
    }
}

impl Default for TaskExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl<F> Executor<F> for TaskExecutor
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, fut: F) {
        background_spawn!("http_executor", fut);
    }
}
