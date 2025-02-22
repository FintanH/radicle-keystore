// This file is part of radicle-link
// <https://github.com/radicle-dev/radicle-link>
//
// Copyright (C) 2019-2020 The Radicle Team <dev@radicle.xyz>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 3 or
// later as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use smol::{io, net::unix::UnixStream};

#[cfg(feature = "ssh-agent")]
fn main() -> io::Result<()> {
    use radicle_keystore::sign::{ssh, Signer, SshAgent};
    use rand::rngs::OsRng;

    smol::block_on(async {
        let sk = ed25519_zebra::SigningKey::new(OsRng {});
        let pk = ed25519_zebra::VerificationKey::from(&sk);

        // This could be a `rad-ssh-add` executable which reads the local key from
        // the filestore (prompting for the password).
        ssh::add_key::<UnixStream>(sk, &[]).await.unwrap();

        println!("connecting to ssh-agent");
        let agent = SshAgent::new(ssh::ed25519::PublicKey(pk.into()))
            .connect::<UnixStream>()
            .await
            .expect("could not connect to ssh-agent");
        println!("asking agent to sign some data");
        let sig = agent
            .sign(b"cooper")
            .await
            .expect("signing via ssh-agent failed");
        println!("verifying signature");
        pk.verify(&ed25519_zebra::Signature::from(sig.0), b"cooper")
            .expect("ssh-agent didn't return a valid signature");
        println!("it worksed");
        Ok(())
    })
}

#[cfg(not(feature = "ssh-agent"))]
fn main() {
    eprintln!("this example requires the `ssh-agent` feature")
}
