# Chama DAO using Polkadot, Chopsticks, and Subsocial

This project is a decentralized autonomous organization (DAO) that allows users to create and manage investment clubs called "chamas". It uses several cutting-edge technologies including Polkadot, Chopsticks, and Subsocial to create a secure and transparent platform for managing investment groups.
Features

    User authentication using Polkadot wallet addresses
    Add members to your chama through the dashboard
    Vote on proposals to invest funds in new ventures
    Disburse funds to approved proposals
    Integration with Subsocial for social networking and content sharing

#Technologies Used
Polkadot

Polkadot is a next-generation blockchain platform that allows for interoperability between different blockchain networks. In this project, we use Polkadot to authenticate users using their wallet addresses.
Chopsticks

Chopsticks is a Rust-based framework for building Substrate-based chains. It provides a lightweight alternative to Substrate, with a focus on simplicity and speed. We use Chopsticks to build the Chama DAO smart contract, which is deployed on the Polkadot network.
Subsocial

Subsocial is a decentralized social network built on the Substrate framework. It allows users to create and share content on a decentralized platform. In this project, we integrate Subsocial into the Chama DAO, allowing members to communicate and share content related to their chama.
How It Works

    Users can log in to the Chama DAO using their Polkadot wallet addresses.
    Once logged in, users can create a new chama and add members through the dashboard.
    Members can propose new investment opportunities to the chama, which are voted on by other members.
    If a proposal is approved, funds are disbursed to the investment opportunity.
    Members can also use Subsocial to communicate and share content related to their chama.

#Getting Started

To run the Chama DAO locally, follow these steps:

    Install the latest version of Rust and Substrate.
    Clone the repository and navigate to the project directory.
    Install dependencies using cargo build.
    Run the project using cargo run.
    
#Project Structures  

ChamaDao/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
├── runtime/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── lib.rs
│   ├── src/
│   │   ├── chama_dao.rs
│   │   └── lib.rs
│   └── target/
├── contract/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── ink_utils/
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── lib.rs
│   │   └── target/
│   ├── src/
│   │   ├── chama_dao.rs
│   │   └── lib.rs
│   └── target/
├── frontend/
│   ├── public/
│   │   ├── index.html
│   │   └── manifest.json
│   ├── src/
│   │   ├── App.js
│   │   ├── components/
│   │   │   ├── ChamaMemberList.js
│   │   │   ├── ProposalForm.js
│   │   │   ├── ProposalList.js
│   │   │   └── VoterList.js
│   │   ├── index.css
│   │   ├── index.js
│   │   └── util.js
│   ├── package-lock.json
│   ├── package.json
│   └── README.md
└── target/


#Contributors

   Moses Ochieng

#License

This project is licensed under the MIT License.
