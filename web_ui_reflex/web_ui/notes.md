# Reflex web ui/app notes

## Create a project

* create project dir

* cd into dir

* run `reflex init`

## Run app

* `reflex run`

* run as debug: `reflex run --loglevel debug`

## Project Structure

* .web: where compiled JS files will be stored. Each Reflex page will compile to a corresponding .js file in .web/pages directory

* Assets: where you can store static assets like images, fonts, and other files

    * to access: `rx.iamge(src="image.png")`

* Main Project: in `{project_name}` dir, location of app logic, in file `{project_name}/{project_name}.py`

* Configuration: `rxconfig.py` file can be used to configure the app

## Configuration

* pass keyword arguments in the Config class of `rxconfig.py`, ex.

```py
# rxconfig.py
import reflex as rx

config = rx.Config(
    app_name="my_app_name",
    # Connect to your own database.
    db_url="postgresql://user:password@localhost:5432/my_db",
    # Change the frontend port.
    frontend_port=3001,
)
```

* config can be overriden by using environment variables, ex. `$ FRONTEND_PORT=3001 reflex run`

* config can be overriden using command line arguments, ex. `$ reflex run --frontend-port 3001`

## Basic Frontend

* the frontend can be broken down into independent reusable components