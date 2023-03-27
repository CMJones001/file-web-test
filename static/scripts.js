let isExpanded = false;

// Function to toggle the table rows
function toggleTableRows() {
    const rowsToHide = document.querySelectorAll('.row-to-hide');
    rowsToHide.forEach((row, index) => {
        if (index >= 2 && !isExpanded) {
            row.classList.add('hidden');
        } else {
            row.classList.remove('hidden');
        }
    });
}

// Hide the rows initially when the page loads
toggleTableRows();

// Add event listener to the button
document.getElementById('expandButton').addEventListener('click', function () {
    isExpanded = !isExpanded;
    toggleTableRows();

    if (isExpanded) {
        this.textContent = 'Hide rows';
    } else {
        this.textContent = 'Show more rows';
    }
});
