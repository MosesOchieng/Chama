const form = document.getElementById("registration-form");
const nameInput = document.getElementById("name-input");
const walletInput = document.getElementById("wallet-address-input");
const btn = document.querySelector(".btn");

form.addEventListener("submit", function (event) {
  event.preventDefault();

  // Get the values of the input fields
  const name = nameInput.value;
  const walletAddress = walletInput.value;

  // Call the Ink smart contract
  const contract = new web3.eth.Contract(CONTRACT_ABI, CONTRACT_ADDRESS);
  const contractCall = contract.methods
    .register(name, walletAddress)
    .send({ from: web3.eth.defaultAccount });

  // Show loading spinner while waiting for contract response
  btn.innerHTML = '<i class="fa fa-spinner fa-spin"></i> Registering...';
  btn.disabled = true;

  // Get the contract response and show success or error message
  contractCall
    .then((result) => {
      alert("Successfully registered!");
      window.location.href = "../Dashboard/index.html";
    })
    .catch((error) => {
      alert("Failed to register. Please try again later.");
      btn.innerHTML = "Submit";
      btn.disabled = false;
    });
});
