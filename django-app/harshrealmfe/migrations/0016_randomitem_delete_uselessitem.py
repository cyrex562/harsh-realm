# Generated by Django 4.2.1 on 2023-05-25 19:04

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('harshrealmfe', '0015_bannedcargo'),
    ]

    operations = [
        migrations.CreateModel(
            name='RandomItem',
            fields=[
                ('id', models.BigAutoField(auto_created=True, primary_key=True, serialize=False, verbose_name='ID')),
                ('dice_roll', models.IntegerField()),
                ('place', models.CharField(max_length=200)),
                ('item', models.TextField()),
            ],
        ),
        migrations.DeleteModel(
            name='UselessItem',
        ),
    ]
