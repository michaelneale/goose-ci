# Goose CI

Run goose as an action.

To use: 


```yaml

      - name: Run Goose Action
        uses: ./.github/actions/goose-action
        with:
          task_request: "make me a time machine in C++"
        env:
          OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}

```

It will do its best to complete the task as part of your workflow (with the tools it has) and return success if it was successful. 

If it succeeds, it will have enhanced your code, and then you could open a pull request, if triggered from a github issue for example. 

TODO: actual examples
