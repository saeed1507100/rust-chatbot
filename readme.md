# Rust Chatbot

This is a Rust-based chatbot application that interacts with the Hugging Face API to provide responses to user queries.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- **Rust**: Make sure you have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  ```

- **Hugging Face API Token**: You need a Hugging Face API token to use their inference API. You can get one by signing up on [Hugging Face](https://huggingface.co/).

## Getting Started

### Clone the Repository

Clone this repository to your local machine:

```bash
git clone https://github.com/yourusername/rust-chatbot.git
cd rust-chatbot
```

### Configure the API Token

Create a `.env` file in the root directory of the project and add your Hugging Face API token:

```plaintext
API_TOKEN=hf_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### Install Dependencies

Install the necessary dependencies:

```bash
cargo build
```

### Running the Application

Run the application using Cargo:

```bash
cargo run
```

### Usage

Once the application is running, you can interact with it via the command line. Type your query and press Enter to get a response.

Example:

```plaintext
Hi, I'm a Rust chatbot! How can I help you today?
--------------------------------------
User: capital of bangladesh

Model Response: capital of bangladesh

The capital of Bangladesh is Dhaka.

It is located in the central part of the country and is the largest city in Bangladesh. Dhaka is a bustling metropolis, with a population of over 20 million people. It is the political, cultural, and economic center of Bangladesh, and is home to many government institutions, universities, and major corporations.

Dhaka was founded in the 7th century as a small trading town

--------------------------------------
```

### Acknowledgements

- [Hugging Face](https://huggingface.co/) for providing the API.
- The Rust community for their support and documentation.
