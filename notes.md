`python manage.py makeimigrations harshrealm_app` -- store changes to models as migration

`python manage.py sqlmigrate harshrealm_app ####` -- take migration commands and display sql

`python manage.py migrate`

add model to `app/admin.py`

`python manage.py runserver`


## Flask Folder Structure

app/blueprints: where blueprints live
app/commands: application-wide commands,ex. scripts to manipulate blueprints
app/enums: application-wide enums
app/ext: application extensions like sqlalchemy, sentry, flask-caching; configure external libraries and invoke them from this place in the app
app/errors: register application error handlers
app/jinja: jinja configuration
app/jinja/__init__.py: jinja mapper definition
app/jinja/context_processor.py: jinja context processor; set keys to be passed to templates
app/jinja/filters.py: custom jinja filters
app/models: application-wide models. other models should be stored in blueprints
app/static: shared static files
app/templates: shared templates; main  layout and error pages
app/app.py main flask file
config: configuration files
instance/config.py: store instance sensitive data; add to .gitignore
configure.py: small utility for initial setup
webservice.ini: uWSGI config for deployment
wsgi.py: entry point to deploy application