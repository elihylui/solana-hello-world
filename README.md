# Simple Hello World Program in Solana

This repo contains a full-stack application which allows users to connect to a Phantom wallet, write a message to a Solana program, read the message and update it. 

On the backend, there is a smart contract deployed to and tested in Solana's dev net. This program is written in Rust and uses the Anchor framework, following the instructions written by [Alchemy](https://docs.alchemy.com/docs/hello-world-solana-program). Note that changes were made to make the program work after Solana's and Anchor's version upgrades. The frontend, on the other hand, uses the React framework. 

To run the program locally, run ```yarn add && yarn dev``` then start your server on ```http://localhost:3000```. 
