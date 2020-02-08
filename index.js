import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const realInput = document.getElementById('real');
        const imaginaryInput = document.getElementById('imaginary');
        const scaleInput = document.getElementById('scale');
        const renderBtn = document.getElementById('render');

        realInput.value = -0.8
        imaginaryInput.value = 0
        scaleInput.value = -0.5

        canvas.imageSmoothingEnabled = true

        renderBtn.addEventListener('click', () => {
            const real = parseFloat(realInput.value) || 0;
            const imaginary = parseFloat(imaginaryInput.value) || 0;
            const scale = parseFloat(scaleInput.value) || 1;
            wasm.draw(ctx, 600, 600, real, imaginary, scale);
        });

        let beforePos = {x:0,y:0};

        canvas.onmousedown = e => {
            beforePos = {
                x: e.clientX,
                y: e.clientY
            }
        };

        canvas.onmouseup = e => {
            const delta = {
                x: e.clientX - beforePos.x,
                y: e.clientY - beforePos.y
            }

            delta.x = delta.x * (Math.pow(10,-scaleInput.value)) / 600;
            delta.y = delta.y * (Math.pow(10,-scaleInput.value)) / 600;

            // console.log(delta)

            realInput.value = parseFloat(realInput.value) - delta.x;
            imaginaryInput.value = parseFloat(imaginaryInput.value) - delta.y;

            const real = parseFloat(realInput.value) || 0;
            const imaginary = parseFloat(imaginaryInput.value) || 0;
            const scale = parseFloat(scaleInput.value) || 1;

            wasm.draw(ctx, 600, 600, real, imaginary, scale);
        }

        canvas.onwheel = ev => {
            ev.preventDefault()
            scaleInput.value = parseFloat(scaleInput.value) - 0.03 * ev.deltaY
            if (parseFloat(scaleInput.value) < -1) {
                scaleInput.value = -1;
            }
            trigger_draw()
        }

        let timer = undefined;
        const trigger_draw = () => {
            if (timer){
                clearTimeout(timer)
            }
            timer = setTimeout(() => {
                const real = parseFloat(realInput.value) || 0;
                const imaginary = parseFloat(imaginaryInput.value) || 0;
                const scale = parseFloat(scaleInput.value) || 1;

                wasm.draw(ctx, 600, 600, real, imaginary, scale)
            }, 300)
        }

        wasm.draw(ctx, 600, 600, realInput.value, imaginaryInput.value,scaleInput.value);
    })
    .catch(console.error);
