# Generated by Django 4.2.1 on 2023-05-18 00:17

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('harshrealmfe', '0006_starshipname'),
    ]

    operations = [
        migrations.CreateModel(
            name='UselessItem',
            fields=[
                ('id', models.BigAutoField(auto_created=True, primary_key=True, serialize=False, verbose_name='ID')),
                ('dice_roll', models.IntegerField()),
                ('item', models.TextField()),
            ],
        ),
    ]
