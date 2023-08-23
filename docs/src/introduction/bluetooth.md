# Bluetooth Stack Comparison

<table style="width: 100%;">
<colgroup>
    <col span = "1">
    <col id="em_tab" span="1" style=visibility:collapse>
    <col id="dr_tab" span="1" style=visibility:collapse>
    <col id="to_tab" span="1" style=visibility:collapse>
    <col id="ri_tab" span="1" style=visibility:collapse>
    <col id="ze_tab" span="1" style=visibility:collapse>
    <col id="fr_tab" span="1" style=visibility:collapse>
  </colgroup>
<tr>
<th></th>
<th>Embassy</th>
<th>Drogue OS</th>
<th>TockOS</th>
<th>RiotOS</th>
<th>ZephyrOS</th>
<th>FreeRTOS</th>
</tr>
<tr>
<td>Number of concurrent links</td>
<td>20</td>
<td>20</td>
<td>-</td>
<td>32 (NimBLE)</td>
<td>unlimited</td>
<td>20-32</td>
</tr>
<tr>
<td>Advertising (Broadcaster)</td>
<td>SoftDevice</td>
<td>SoftDevice</td>
<td>TockOS Stack</td>
<td>NimBLE/ Skald</td>
<td>Zephyr Stack</td>
<td>SoftDevice/ NimBLE</td>
</tr>
<tr>
<td>Connecting (central)</td>
<td>SoftDevice</td>
<td>SoftDevice</td>
<td>-</td>
<td>NimBLE</td>
<td>Zephyr Stack</td>
<td>SoftDevice/ NimBLE</td>
</tr>
<tr>
<td>Scanning (observer)</td>
<td>SoftDevice</td>
<td>SoftDevice</td>
<td>TockOS Stack</td>
<td>NimBLE</td>
<td>Zephyr Stack</td>
<td>SoftDevice/ NimBLE</td>
</tr>
<tr>
<td>Connectable (peripheral)</td>
<td>SoftDevice</td>
<td>SoftDevice</td>
<td>-</td>
<td>NimBLE</td>
<td>Zephyr Stack</td>
<td>SoftDevice/ NimBLE</td>
</tr>
<tr>
<td>Over-air device firmware updates</td>
<td>SoftDevice</td>
<td>SoftDevice</td>
<td>-</td>
<td>-</td>
<td>-</td>
<td>-</td>
</tr>
<tr>
<td>Asynchronous, event-driven behaviour</td>
<td>SoftDevice</td>
<td>SoftDevice</td>
<td>-</td>
<td>-</td>
<td>-</td>
<td>-</td>
</tr>
<tr>
<td>Implemented in Rust</td>
<td>-</td>
<td>-</td>
<td>TockOS Stack</td>
<td>-</td>
<td>-</td>
<td>-</td>
</tr>
<tr>
<td>Implemented in C</td>
<td>SoftDevice</td>
<td>SoftDevice</td>
<td>-</td>
<td>NimBLE</td>
<td>Zephyr Stack</td>
<td>SoftDevice/ NimBLE</td>
</tr>
<tr>
<td>Bluetooth Mesh</td>
<td>(SoftDevice)</td>
<td>(SoftDevice)</td>
<td>-</td>
<td>NimBLE</td>
<td>Zephyr Stack</td>
<td>NimBLE</td>
</tr>
<tr>
<td>L2CAP Connections</td>
<td>SoftDevice</td>
<td>SoftDevice</td>
<td>-</td>
<td>NimBLE</td>
<td>Zephyr Stack</td>
<td>SoftDevice/ NimBLE</td>
</tr>
</table>

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
        var em_link = document.getElementById('em_tab');
        if (!embassy.checked) {
            em_link.style.visibility = 'collapse';
        } else {
            em_link.style.visibility = 'visible';
        }

        var dr_link = document.getElementById('dr_tab');
        if (!drogue.checked) {
            dr_link.style.visibility = 'collapse';
        } else {
            dr_link.style.visibility = 'visible';
        }

        var to_link = document.getElementById('to_tab');
        if (!tockos.checked) {
            to_link.style.visibility = 'collapse';
        } else {
            to_link.style.visibility = 'visible';
        }

        var ri_link = document.getElementById('ri_tab');
        if (!riotos.checked) {
            ri_link.style.visibility = 'collapse';
        } else {
            ri_link.style.visibility = 'visible';
        }

        var ze_link = document.getElementById('ze_tab');
        if (!zephyros.checked) {
            ze_link.style.visibility = 'collapse';
        } else {
            ze_link.style.visibility = 'visible';
        }

        var fr_link = document.getElementById('fr_tab');
        if (!freertos.checked) {
            fr_link.style.visibility = 'collapse';
        } else {
            fr_link.style.visibility = 'visible';
        }
    }
</script>    