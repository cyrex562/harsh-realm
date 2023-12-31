"""empty message

Revision ID: 0702d0d9a9db
Revises: d12bf7df0e43
Create Date: 2023-08-29 14:20:30.022692

"""
from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa
import sqlmodel

# revision identifiers, used by Alembic.
revision: str = '0702d0d9a9db'
down_revision: Union[str, None] = 'd12bf7df0e43'
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    op.add_column('sessionmessage', sa.Column('message_type', sqlmodel.sql.sqltypes.AutoString(), nullable=False))
    # ### end Alembic commands ###


def downgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    op.drop_column('sessionmessage', 'message_type')
    # ### end Alembic commands ###
