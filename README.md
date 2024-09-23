# Goose CI

Run goose as an action in CI. 

The basics: 

```yaml

      - name: Run Goose Action
        uses: ./.github/actions/goose-action
        with:
          task_request: "make me a time machine in C++"
          validation: "run make test to check it passes"
          create_pr: true
        env:
          OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}

```

It will do its best to complete the task as part of your workflow (with the tools it has). If it succeeds in this case a PR will be opened based on the changes it made. If not - no PR results (or the job fails!).

## Example workflow

To use this in your workflow, it is usually best to trigger off a labelled issue (but doesn't have to).
In this repo there is [an example workflow](.github/workflows/goose-example-workflow.yml) which is triggered when you open an issue on this repo, and label it as "goose" (if it can solve it, a PR will result linked to that issue).
The issue serves as input direction for goose. Note the `validation` parameter in the goose action, that is important so it knows how to check its work as it goes (and if it thinks it has ultimately succeeded).

## Advanced usage and customising. 

Note due to github limitations, PRs opened by the default GITHUB_TOKEN won't trigger workflows (but you can use a personal access token if you do want it to do that, or have it trigger downstream workflows). 

If you want to customise how it opens a PR, you can set `create_pr: false`, and then use https://github.com/peter-evans/create-pull-request in your own workflow. This will let you set the token to trigger workflows and more.

