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

It will do its best to complete the task as part of your workflow (with the tools it has). If it succeeds in this case a PR will be opened based on the changes it made. If not - no PR results (and that job fails)
## Example workflow

To use this in your workflow, it is usually best to trigger off a labelled issue (but doesn't have to).
In this repo there is [an example workflow](.github/workflows/goose-example-workflow.yml) which is triggered when you open an issue on this repo, and label it as "goose" (if it can solve it, a PR will result linked to that issue).
The issue serves as input direction for goose. Note the `validation` parameter in the goose action, that is important so it knows how to check its work as it goes (and if it thinks it has ultimately succeeded).
