# Commit Crafter

## Installation

```bash
cargo install --locked commit-crafter
```

use homebrew

```bash
brew install commit-crafter
```

In the git project, install the prepare-commit-msg hook and set up the OpenAI API key to use it. If it is the first time installing and using it.

```bash
commit-crafter initstall

commit-crafter config set openai_ai_key <your key>
```

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
- [ ] Add more tests
- [ ] Improve README.md