## SeaORM Database models 
These are automatically generated with some manual edits for runtime defaults. Be careful when regenerating and make sure to generate to a new directory and cross reference the changes. Guidelines on how to do this are below and some steps can be skipped as long as you pay attention to what you are doing (eg. model has not changed at all).

## How to generate
1. Set `DATABASE_URL` to a proper database URI.
	* Use postgres for this process. We support multiple databases but enum types can only be generated when using Postgres.
2. Either run the application once to automatically run migrations or run `sea-orm-cli migrate up`.
3. Run `sea-orm-cli generate entity -o entities`.
4. Cross reference changes in the `entities` folder and the `database/entity` folder and make sure you keep `ActiveModelBehavior`s on each file as-is since they specify the ID generator and applied defaults on each model.
5. Delete `entities` and make sure everything works properly (tests coming soon).