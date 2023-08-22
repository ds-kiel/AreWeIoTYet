# Bluetooth Stack Comparison
| Chacteristics on BLE Stack | Embassy | Drogue OS | TockOS | RiotOS | ZephyrOS | FreeRTOS |  
|---|---|---|---|---|---|---|  
| Number of concurrent links | 20 | 20 | - | 32 (NimBLE) | unlimited | 20-32 |
| Advertising (Broadcaster) | SoftDevice | SoftDevice | TockOS Stack | NimBLE/ Skald | Zephyr Stack | SoftDevice/ NimBLE |
|Connecting (central) | SoftDevice | SoftDevice |-| NimBLE | ZephyrStack | SoftDevice/NimBLE |
| Scanning (Observer) | SoftDevice | SoftDevice | TockOS Stack | NimBLE | Zephyr Stack | SoftDevice/ NimBLE|
|Connectable (peripheral) | SoftDevice | SoftDevice |-| NimBLE | ZephyrStack | SoftDevice/NimBLE |
Over-air device firmware updates | SoftDevice | SoftDevice |-|-|-|-|
| Asynchronous, event-driven behaviour | SoftDevice | SoftDevice |-|-|-|-|
|Implemented in Rust |-|-| TockOS Stack |-|-|-|
| Implemented in C | SoftDevice | SoftDevice |-| NimBLE | ZephyrStack | SoftDevice/ NimBLE |
| Bluetooth Mesh | (SoftDevice) | (SoftDevice) |-| NimBLE | ZephyrStack | NimBLE |
L2CAP Connections | SoftDevice | SoftDevice |-| NimBLE | ZephyrStack | SoftDevice/ NimBLE |

<fieldset id="Checkboxes">
  <legend>Choose the OS's to be compared </legend>
  <div>
    <input type="checkbox" id="embassy" name="interest" value="embassy"/>
    <label for="embassy">Embassy</label>
  </div>
  <div>
    <input type="checkbox" id="drogue" name="interest" value="drogue" />
    <label for="drogue">Drogue OS</label>
  </div>
  <div>
    <input type="checkbox" id="tockos" name="interest" value="tockos"/>
    <label for="tockos">TockOS</label>
  </div>
  <div>
    <input type="checkbox" id="riotos" name="interest" value="riotos" />
    <label for="riotos">RiotOS</label>
  </div>
  <div>
    <input type="checkbox" id="zephyros" name="interest" value="zephyros"/>
    <label for="zephyros">ZephyrOS</label>
  </div>
  <div>
    <input type="checkbox" id="freertos" name="interest" value="freertos" />
    <label for="freertos">FreeRTOS</label>
  </div>
</fieldset>

<div id="em-elric"> Embassy </div>
<div id="dr-elric"> Drogue </div>
<div id="to-elric"> TockOS </div>
<div id="ri-elric"> RiotOS </div>
<div id="ze-elric"> ZephyrOS </div>
<div id="fr-elric"> FreeRTOS </div>

<script>
    const embassy = document.querySelector('#embassy');
    embassy.addEventListener("change", updateDisplay);
    const drogue = document.querySelector('#drogue');
    drogue.addEventListener("change", updateDisplay);
    const tockos = document.querySelector('#tockos');
    tockos.addEventListener("change", updateDisplay);
    const riotos = document.querySelector('#riotos');
    riotos.addEventListener("change", updateDisplay);
    const zephyros = document.querySelector('#zephyros');
    zephyros.addEventListener("change", updateDisplay);
    const freertos = document.querySelector('#freertos');
    freertos.addEventListener("change", updateDisplay);

    function updateDisplay() {
        var em_link = document.getElementById('em-elric');
        if (embassy.checked) {
            em_link.style.visibility = 'hidden';
        } else {
            em_link.style.visibility = 'visible';
        }

        var dr_link = document.getElementById('dr-elric');
        if (drogue.checked) {
            dr_link.style.visibility = 'hidden';
        } else {
            dr_link.style.visibility = 'visible';
        }

        var to_link = document.getElementById('to-elric');
        if (tockos.checked) {
            to_link.style.visibility = 'hidden';
        } else {
            to_link.style.visibility = 'visible';
        }

        var ri_link = document.getElementById('ri-elric');
        if (riotos.checked) {
            ri_link.style.visibility = 'hidden';
        } else {
            ri_link.style.visibility = 'visible';
        }

        var ze_link = document.getElementById('ze-elric');
        if (zephyros.checked) {
            ze_link.style.visibility = 'hidden';
        } else {
            ze_link.style.visibility = 'visible';
        }

        var fr_link = document.getElementById('fr-elric');
        if (freertos.checked) {
            fr_link.style.visibility = 'hidden';
        } else {
            fr_link.style.visibility = 'visible';
        }
    }
</script>    