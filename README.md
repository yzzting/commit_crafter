# Commit Crafter

## Installation

```bash
cargo install --locked commit_crafter
```

In the git project, install the prepare-commit-msg hook and set up the OpenAI API key to use it. If it is the first time installing and using it.

```bash
commit_crafter install
```

After executing the installation command, you must first set up a key in order to use it normally.

```bash
commit_crafter config set openai_api_key <your key>
```

## Options

```bash
// openai api key
commit_crafter config set openai_api_key <your key>

// openai url
commit_crafter config set openai_url <your url>

// openai model
commit_crafter config set openai_model <your model>

// prompt language
commit_crafter config set user_language <your language>

// get config options
commit_crafter config get <option>

// get all config options
commit_crafter config list
```

Language List:
| Language | Code |
| --- | --- |
| English | en |
| Japanese | jp |
| 简体中文 | zh |
| 繁体中文 | zh_tw |

The default file path is $HOME/.config/commit_crafter/config.toml

## Usage

After correctly installing the hook, execute "git commit -a" in the git project. In the temporary Vim editor interface that opens, there will be generated commit information. The prerequisite is that all files have been staged for commit.

```bash
# prerequisites
git add . // or git add <file>

git commit -a
```

## Configuration

## To Do

- [ ] Add more options to customize the commit message
- [ ] Support more AI models
- [ ] Support more languages
- [X] Add more tests
- [ ] Improve README.md
