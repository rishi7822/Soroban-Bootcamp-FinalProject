// Define the list of poll options (you can fetch this dynamically from the contract)
const pollOptions = ["Option 1", "Option 2", "Option 3"];

// Function to dynamically populate the poll options
function populatePollOptions() {
    const pollOptionsDiv = document.getElementById('poll-options');
    pollOptions.forEach((option, index) => {
        const input = document.createElement('input');
        input.type = 'number';
        input.min = '0';
        input.id = 'votes-' + index;
        input.placeholder = 'Votes for ' + option;
        const label = document.createElement('label');
        label.textContent = option;
        label.appendChild(input);
        pollOptionsDiv.appendChild(label);
    });
}

// Function to record votes
function recordVotes() {
    const votes = {};
    pollOptions.forEach((option, index) => {
        const votesInput = document.getElementById('votes-' + index);
        if (votesInput.value.trim() !== '') {
            votes[option] = parseInt(votesInput.value.trim());
        }
    });
    
    // Send votes to the smart contract using Web3 or other suitable mechanism
    console.log('Recorded votes:', votes);
}

// Populate poll options when the page loads
window.onload = populatePollOptions;
