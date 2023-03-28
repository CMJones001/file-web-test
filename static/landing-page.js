let progress = 0;

const increaseButton = $('#progress-button');
increaseButton.click(() => {
    progress += 10;
    $('#base-progress').progress({percent: progress});
});
