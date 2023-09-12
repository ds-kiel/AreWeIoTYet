# Bluetooth Stack Comparison
We compare five operating systems and their Bluetooth stacks on eleven characteristics. The intention is to enable a feeling of what the different operating systems are capable of. Afterward, we give a short summary of the different stacks used.

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
    <input type="checkbox" id="tock" name="interest" value="tock" checked/>
    <label for="tock">tock</label>
  </div>
  <div>
    <input type="checkbox" id="riot" name="interest" value="riot" checked/>
    <label for="riot">RIOT</label>
  </div>
  <div>
    <input type="checkbox" id="zephyr" name="interest" value="zephyr" checked/>
    <label for="zephyr">Zephyr</label>
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
<th>tock</th>
<th>RIOT</th>
<th>Zephyr</th>
</tr>
<tr>
<td>Bluetooth Stack</td>
<td>SoftDevice</td>
<td>SoftDevice</td>
<td>tock Stack</td>
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

Some of the operating systems use the same underlying Bluetooth stack. Thus we only have four different Bluetooth stacks:
1. SoftDevice
2. NimBLE
3. Zephyr Stack
4. tock Stack

### SoftDevice
The [SoftDevice](https://infocenter.nordicsemi.com/index.jsp?topic=%2Fsds_s140%2FSDS%2Fs1xx%2Fble_protocol_stack%2Fble_protocol_stack.html) includes a BLE protocol stack that is compliant with Bluetooth 5.1 (Host & Controller). It is built and provided by Nordic Semiconductor, the  producer of the nRF52840 DK board we targeted. Thus, it provides the full support those boards are capable of. Sadly, it is not Open Source and written in C. So the usage of this stack makes it impossible to build an application purely based on Rust. But, it clearly enables the best usage of the hardware.

### NimBLE
[Apache NimBLE](https://github.com/apache/mynewt-nimble) is a Bluetooth 5.4 compliant Bluetooth stack. It is part of the Apache Mynewt project which "is an open source operating system for tiny embedded devices" ([Apache Mynewt](https://github.com/apache/mynewt-core)). NimBLE approaches to replace the SoftDevice and thus supports multiple Nordics chipsets. It is also written in C but it is open source which might make it favorable over Nordics SoftDevice.

### Zephyr Stack
The [Zephyrs Bluetooth Stack](https://docs.zephyrproject.org/latest/connectivity/bluetooth/overview.html) is Bluetooth 5.3 compliant. It is part of the zephyrproject or the Zephyr RTOS which is open source and programmed in C. While the stack overall is fine, bringing it together with Rust is a hassle as described later in [Zephyr](../zephyr/README.md).

### tock Stack
The [tock Bluetooth Stack](https://github.com/tock/tock/blob/master/doc/BluetoothLEStack.md) is part of [tock](https://github.com/tock/tock), a secure open source operating system for embedded devices which is completely written in Rust. Sadly, the Tock OS Bluetooth Stack is still quite minimal, which results in not being able to connect. This limits the capability of this stack to advertising and scanning.

<!-- Javascript part to toggle the visibility of the columns -->
<!-- Yes, iterating through the checkboxes would have been more beautiful -->
<!-- Yes, iterating through the columns would also have been more beautiful -->
<script>
    const embassy = document.querySelector('#embassy');
    embassy.addEventListener("change", updateDisplay);
    const drogue = document.querySelector('#drogue');
    drogue.addEventListener("change", updateDisplay);
    const tock = document.querySelector('#tock');
    tock.addEventListener("change", updateDisplay);
    const riot = document.querySelector('#riot');
    riot.addEventListener("change", updateDisplay);
    const zephyr = document.querySelector('#zephyr');
    zephyr.addEventListener("change", updateDisplay);

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
        if (!tock.checked) {
            to_link.style.visibility = 'collapse';
        } else {
            to_link.style.visibility = 'visible';
        }

        var ri_link = document.getElementById('ri_tab');
        if (!riot.checked) {
            ri_link.style.visibility = 'collapse';
        } else {
            ri_link.style.visibility = 'visible';
        }

        var ze_link = document.getElementById('ze_tab');
        if (!zephyr.checked) {
            ze_link.style.visibility = 'collapse';
        } else {
            ze_link.style.visibility = 'visible';
        }
    }
</script>    