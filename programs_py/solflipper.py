# solflipper
# Built with Seahorse v0.2.6

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


class Flipper(Account):
    state: bool
    owner: Pubkey


@instruction
def initialize(owner: Signer, flipper: Empty[Flipper]):
    flipper = flipper.init(
        payer=owner,
        seeds=['Flipper', owner]
    )
    flipper.owner = owner.key()


@instruction
def flip(owner: Signer, flipper: Flipper):
    assert owner.key() == flipper.owner, 'This is not your lightswitch!'
    flipper.state = not flipper.state
