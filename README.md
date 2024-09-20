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

It will do its best to complete the task as part of your workflow (with the tools it has) and then if it succeeds, it succeeds. 
The output could be used to open a pull request, if triggered from a github issue for example. 

TODO: actual examples
