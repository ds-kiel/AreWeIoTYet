# Bluetooth Stack Comparison
We compare six operating systems and their bluetooth stacks on eleven characteristics. The intention is to enable a feeling on what the different operating systems are capable of. Afterwards, we give a short summary on the different stacks used.

<!-- Checkboxes to toggle the visibility of the columns of the table -->
<fieldset id="Checkboxes">
  <legend>Choose the OS's to be compared </legend>
  <div>
    <input type="checkbox" id="embassy" name="interest" value="embassy"checked/>
    <label for="embassy">Embassy</label>
  </div>
  <div>
    <input type="checkbox" id="drogue" name="interest" value="drogue" checked/>
    <label for="drogue">Drogue OS</label>
  </div>
  <div>
    <input type="checkbox" id="tockos" name="interest" value="tockos" checked/>
    <label for="tockos">TockOS</label>
  </div>
  <div>
    <input type="checkbox" id="riotos" name="interest" value="riotos" checked/>
    <label for="riotos">RiotOS</label>
  </div>
  <div>
    <input type="checkbox" id="zephyros" name="interest" value="zephyros" checked/>
    <label for="zephyros">ZephyrOS</label>
  </div>
</fieldset>

<!-- The actual table -->
<table text-align="left" style="width: 100%;">
<!-- Assigning ids to each column to be able to collapse them with javascript later -->
<colgroup>
    <col span = "1">
    <col id="em_tab" span="1" style=visibility:visible>
    <col id="dr_tab" span="1" style=visibility:visible>
    <col id="to_tab" span="1" style=visibility:visible>
    <col id="ri_tab" span="1" style=visibility:visible>
    <col id="ze_tab" span="1" style=visibility:visible>
  </colgroup>
<!-- Content of the table -->
<tr>
<th></th>
<th>Embassy</th>
<th>Drogue OS</th>
<th>TockOS</th>
<th>RiotOS</th>
<th>ZephyrOS</th>
</tr>
<tr>
<td>Bluetooth Stack</td>
<td>SoftDevice</td>
<td>SoftDevice</td>
<td>TockOS Stack</td>
<td>NimBLE</td>
<td>Zephyr Stack</td>
</tr>
<tr>
<td>Number of concurrent links</td>
<td>20</td>
<td>20</td>
<td><i class="fa fa-xmark"></i></td>
<td>32</td>
<td>unlimited</td>
</tr>
<tr>
<td>Advertising (Broadcaster)</td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
</tr>
<tr>
<td>Connecting (central)</td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
</tr>
<tr>
<td>Scanning (observer)</td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
</tr>
<tr>
<td>Connectable (peripheral)</td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
</tr>
<tr>
<td>Over-air device firmware updates</td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-xmark"></i></td>
</tr>
<tr>
<td>Asynchronous, event-driven behaviour</td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-xmark"></i></td>
</tr>
<tr>
<td>Implemented in Rust</td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-xmark"></i></td>
</tr>
<tr>
<td>Implemented in C</td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
</tr>
<tr>
<td>Bluetooth Mesh</td>
<td>Maybe</i></td>
<td>Maybe</td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
</tr>
<tr>
<td>L2CAP Connections</td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-xmark"></i></td>
<td><i class="fa fa-check"></i></td>
<td><i class="fa fa-check"></i></td>
</tr>
</table>  

Some of the operating systems use the same underlying bluetooth stack. Thus we only have four different bluetooth stacks:
1. SoftDevice
2. NimBLE
3. Zephyr Stack
4. TockOS Stack

### SoftDevice

### NimBLE

### Zephyr Stack

### TockOS Stack

<!-- Javascript part to toggle the visibility of the columns -->
<!-- Yes, iterating through the checkboxes would have been more beautiful -->
<!-- Yes, iterating through the columns would also have been more beautiful -->
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
    }
</script>    