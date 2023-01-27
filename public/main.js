async function init() {
    let rustApp = null;

    try {
        //we import the wasm module in the rustApp variable from the pkg variable that wasm-pack creates for us
        rustApp = await import('../pkg');
    } catch (e) {
        //if it fails to import the module, we just return the function
        return;
    }

    //we create a reference to the input element to listen for events
    const input = document.getElementById('upload');
    //we also want to store the file locally before we send it to the WASM module
    //we could do it with the input.files[0] expression, but we prefer to send rust the data as a string
    //and not as a binary stream
    const fileReader = new FileReader();

    input.addEventListener('change', () => {
        fileReader.readAsDataURL(input.files[0]);
    })

    //we fire off this event when the FileReader class has finished reading the binary stream and converted it to base64
    fileReader.onloadend = () => {
        //we store the result as base64, and we cleanup the string of headers added by the fileResult method. We only want the base64
        const base64 = fileReader.result.replace(
            /^data:image\/(png|jpg|jpeg);base64,/, ''
        );
        rustApp.grayscale(base64);
    }
}

init();