---
title: "Fine-tune a Language Model on x86 CPUs using Bazel and NativeLink"
tags: ["news", "blog-posts"]
image: https://github.com/user-attachments/assets/ddfb5684-327b-4af9-9618-be707eab894f
slug: finetune-with-bazel-nativelink
pubDate: 2025-05-15
readTime: 20 minutes
---

## Introduction

The future of AI development belongs not necessarily to those with the most powerful infrastructure but to those who can extract maximum value from available resources. This tutorial emphasizes CPU-based fine-tuning demonstrating that with intelligent resource management through NativeLink, impressive results can be achieved without expensive GPU or TPU infrastructure. As compute becomes increasingly costly, competitive advantage will shift toward teams that optimize resource efficiency rather than those deploying state-of-the-art hardware.

This guide demonstrates how to establish an optimized AI development pipeline by integrating several key technologies:<br>
1\. **Bazel Build System**: For efficient repository management. A repository managed by Bazel allows your team to work in a unified codebase while maintaining clean separation of concerns.<br>
2\. **NativeLink**: A remote execution system hosted in your cloud. With NativeLink's remote execution capabilities, you can leverage your cloud resources optimally without wasteful duplication of work.<br>
3\. **Hugging Face Transformers**: For integrating with the rich ecosystem of open-source models that you can run locally or deploy anywhere. The transformers library also provides a sophisticated caching mechanism for optimizing loading model weights.


## Setting Up Your Repository With Bazel

Bazel is a build system designed for repositories that allows you to organize code into logical components while maintaining dependency relationships. For AI workloads, this is particularly valuable as it lets you separate model definitions, data processing pipelines, training code, and inference services.

### Prerequisites

1\. A recent version of Bazel ([installation instructions](https://bazel.build/install)). <br>
2\. NativeLink [Cloud Account](https://app.nativelink.com/) (it’s free to get started, and secure) or [NativeLink 0.6.0](https://github.com/TraceMachina/nativelink/releases/tag/v0.6.0) (Apache-licensed, open source is hard-mode)

### Initial Setup

First, let's download all the files. From the folder where you want to download the files, run the following commands:

<div style="max-height: 250px; max-width: 50vw; overflow: auto;">

```bash
# Clone the entire repository
git clone https://github.com/TraceMachina/nativelink-blogs.git

# Navigate to the subdirectory
cd nativelink-blogs/finetuning_on_cpu
```
</div>


Here's a description of some of the files:<br>
1\. `README.md` - Instructions on how to connect/use NativeLink Cloud and how to run the code locally as well as remotely<br>
2\. `requirements.lock` - Ensures consistent Python dependencies across all environments<br>
3\. `.bazelrc` - Main Bazel configuration file setting global options for hermetic builds and remote execution<br>
4\. `MODULE.bazel` - Configures the project as a Bazel module, tells Bazel we'll need Python, `pip` and CPU-only PyTorch, and manages external dependencies<br>
5\. `pyproject.toml` - Python package configuration specifying dependencies and development tools<br>
6\. `BUILD.bazel` (root) - Root build file defining lock targets for Python dependency management<br>
7\. `platforms/BUILD` - Defines Linux x86_64 execution platform running in Ubuntu 24.04 for remote builds<br>
8\. `training/BUILD` - Defines the model training targets and their dependencies<br>
9\. `training/main.py` - Main script that handles fine-tuning of language models on CPU using efficient training techniques<br>


## Important Note About Bazel’s Remote Execution Support

Bazel supports remote execution for building (compiling) and testing, but `bazel run` uses the host platform as its target platform, meaning the executable will be invoked locally rather than on remote machines. To execute binaries on remote servers, a workaround is to design tests that execute the binary, effectively leveraging `bazel test` which runs on the target platform.

**DRAWBACK:**

Logging and print statements that track progress (like "Starting to fine tune model" or "Exiting this function") behave differently in remote execution. Unlike local runs where these appear in real-time, remote testing collects all logs on the server and only displays them after test completion when control returns to the local machine. The expected output is still fully preserved - just delayed until the process finishes running.


## Aside - Configuring Remote Execution For ARM (Apple Silicon)


Most cloud infrastructure (including NativeLink) runs on x86_64 processors, while Apple Silicon Macs use ARM64. Dependencies such as PyTorch are built for specified architectures and setting up Bazel to handle platform-specific dependencies is complex.

If you want to run this code and you only have a Mac, the simplest way would be to run this code via a cloud-based Linux VM (GCP/AWS). If you don't want to use a cloud server, you could create a minimal docker container for x86_64 (`FROM --platform=linux/amd64 ubuntu:22.04` with just `curl` and `Bazelisk` installed) with **Rosetta enabled** and run from your Mac's terminal using this container. However, we highly recommend against this approach; the correct approach here would be to use toolchain transitions from a local mac platform to the remote Linux runner, but that's outside of the scope of this article.


## The NativeLink Difference

To demonstrate NativeLink’s efficacy, consistency, and reliability, we ran the same fine-tuning job on the CPU of an M1 Pro MacBook Pro, the free version of Google Colab on CPU, and [NativeLink](https://github.com/TraceMachina/nativelink), which is free and open-source. We executed the fine-tuning task 5 times and this is what we observed:

1\. The Mac: the quickest run took 18 minutes while the slowest/longest took 20 minutes

2\. Free version of Google Colab: the quickest run took 10 minutes while the slowest/longest took 20 minutes. The execution time was widely varied. We suspect varying traffic on Google’s servers and how Colab allocates its compute resources played a part in this variability.

3\. Free NativeLink: the quickest run took 4 minutes of compute time while the slowest/longest took 6 minutes. NativeLink Cloud provided the quickest execution times by far.

<img src="https://github.com/user-attachments/assets/c2bf8e0e-1500-4ee7-a0f7-6b4ca2196257" width="1000" alt="Model Fine-Tuning Times">


## Conclusion: Optimizing AI Development Through Resource Efficiency

As demonstrated through this tutorial, the integration of Bazel's repository management, NativeLink's CPU-optimized remote execution, and Hugging Face's transformers library creates a development ecosystem that prioritizes computational efficiency over raw processing power. This approach addresses several critical challenges facing modern AI teams:

1\. **Resource Optimization**: By leveraging NativeLink's intelligent scheduling and optimization on CPU infrastructure, teams can achieve impressive fine-tuning results without the capital expenditure of specialized GPU/TPU hardware.<br>
2\. **Strategic Advantage**: This CPU-focused approach provides a competitive edge through efficient resource utilization, enabling teams to allocate budget toward innovation rather than hardware acquisition.<br>
3\. **Sustainable Scaling**: As models grow in size and complexity, the ability to efficiently distribute workloads across existing CPU infrastructure provides a more sustainable path to scale than continuously upgrading to the latest accelerators.<br>

For forward-thinking AI teams, this infrastructure stack represents a shift from the "bigger is better" hardware arms race toward thoughtful resource utilization. The competitive advantage increasingly belongs to those who can extract maximum value from available compute rather than those who deploy more powerful hardware.

The journey from experimental AI projects to production-grade systems demands both technical sophistication and resource awareness. By adopting this CPU-optimized approach with Bazel and NativeLink, your team can focus less on infrastructure limitations and more on the creative potential of fine-tuned models—developing applications that deliver genuine value while maintaining computational efficiency.
