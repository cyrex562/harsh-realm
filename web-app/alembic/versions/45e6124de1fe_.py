"""empty message

Revision ID: 45e6124de1fe
Revises: f472bf6f8d4b
Create Date: 2023-08-29 12:00:22.038638

"""
from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa
import sqlmodel

# revision identifiers, used by Alembic.
revision: str = '45e6124de1fe'
down_revision: Union[str, None] = 'f472bf6f8d4b'
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    # op.add_column('sessionmessage', sa.Column('game_session_id', sa.Integer(), nullable=False))
    op.create_foreign_key(None, 'sessionmessage', 'gamesession', ['game_session_id'], ['id'])
    # ### end Alembic commands ###


def downgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    # op.drop_constraint(None, 'sessionmessage', type_='foreignkey')
    op.drop_column('sessionmessage', 'game_session_id')
    # ### end Alembic commands ###
