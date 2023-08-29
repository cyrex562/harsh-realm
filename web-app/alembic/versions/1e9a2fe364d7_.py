"""empty message

Revision ID: 1e9a2fe364d7
Revises: 
Create Date: 2023-08-29 10:40:09.054999

"""
from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa
import sqlmodel

# revision identifiers, used by Alembic.
revision: str = '1e9a2fe364d7'
down_revision: Union[str, None] = None
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    op.create_table('dicerollresult',
    sa.Column('id', sa.Integer(), nullable=False),
    sa.Column('num_dice', sa.Integer(), nullable=False),
    sa.Column('die_size', sa.Integer(), nullable=False),
    sa.Column('modifier', sa.Integer(), nullable=False),
    sa.Column('mod_sign', sqlmodel.sql.sqltypes.AutoString(), nullable=False),
    sa.Column('rolls', sa.Integer(), nullable=False),
    sa.Column('total', sa.Integer(), nullable=False),
    sa.PrimaryKeyConstraint('id')
    )
    op.create_table('textline',
    sa.Column('id', sa.Integer(), nullable=False),
    sa.Column('text', sqlmodel.sql.sqltypes.AutoString(), nullable=False),
    sa.Column('text_type', sa.Enum('NONE', 'COMMAND_INPUT', 'COMMAND_OUTPUT', 'HELP', 'ERROR', name='texttype'), nullable=False),
    sa.PrimaryKeyConstraint('id')
    )
    # ### end Alembic commands ###


def downgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    op.drop_table('textline')
    op.drop_table('dicerollresult')
    # ### end Alembic commands ###