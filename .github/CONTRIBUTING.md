# Introduction

First of all, thank you for considering contributing to this project. Hibana is made up of many contributions by the people like you.

Development with common guidelines makes your reports, requests, claims and changes easy to communicate to other developers. Only when you follow these guidelines can you contribute Hibana.

Hibana is an open source project and we love  **Free** (in other words, **Libre**). You can contribute to Hibana in many ways. Submitting bug reports and feature requests, creating PRs, improving documentation, writing tutorials or introducing Hibana are all examples of helpful contributions.

Again, development with common guidelines means less work for you. For instance, please use templates if submitting issues or creating PRs.

Please also read this [CODE OF CONDUCT](https://github.com/student-kyushu/hibana/blob/master/.github/CODE_OF_CONDUCT.md).

## Your First Contribution

Unsure where to begin contributing to Hibana? You can start by looking for help-wanted issues. (If you're a member of KSA and want to develop with us, please tell the managers that on Discord.)

Working on your first Pull Request? You can learn how from this *free* series [How to Contribute to an Open Source Project on GitHub](https://egghead.io/series/how-to-contribute-to-an-open-source-project-on-github).

Don't hesitate to contribute and ask for help; everyone is a beginner at first.🐣

## Pull Requests

We love PRs! Here's a quick guide:

1. Fork this repository
2. Happy coding
3. Make all the tests pass (Run `cargo test`)
4. Commit your changes (Please use an appropriate commit prefix)
5. Push to your fork and create a PR
6. Ask someone to review
7. If reviewer approves your PR, it will be tried to merge
8. When there is no problem, your code will be merged right away
9. Great! Thanks for contributing!

Except for minor or urgent changes, PRs are reviewed by another person. To ask someone to review, you can add an `r?` to the message. We have a smart and cute bot, @KoujiroFrau, that will automatically assign a reviewer. For example, you can ask Huyuumi, core developer, to review as follow:

```Markdown
r? @JohnTitor
```

After someone has reviewed and approved your PR, they will leave a comment like this:

```Markdown
@KoujiroFrau r+
```

This tells @KoujiroFrau that your PR has been approved, she queues the PR into the approved queue. If there is no active item, she tries to merge the PR into the latest upstream and run CI on the special branch used for auto testing. When the CI statuses are all green, she will merge your code into `master` and close the PR.

Please explain your changes! Reviewers don't have telepathy.😢

Making smaller PRs is a good way to reduce review time. Please make PRs only one feature or change per PR.

### Commit Prefix

Recommend to use these:

* feat: New features
* fix: Bug fixes
* docs: Changes documentation
* refactor: Changes that neither fixes a bug nor adds a new feature
* perf: Improves performance
* test: Changes test/* files
* minor: Minor changes(e.g. fixes typo)

## Issues

If you find a security vulnerability, do **NOT** open an issue. Email contact@student-kyushu.org instead.

Feature requests and bug reports are filed in https://github.com/student-kyushu/hibana/issues. Before submitting a new issue, please search for similar issues. If somebody has encountered similar bug or wanted similar feature, please add your reaction or comment to the issues.💓

## Community

Hibana is developed with the support and involvement of KSA. If you're interested in us, please visit https://student-kyushu.org.
