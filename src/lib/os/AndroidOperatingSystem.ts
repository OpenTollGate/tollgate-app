import type IOperatingSystem from "$lib/os/IOperatingSystem";
import type { ConnectedNetworkInfo } from "$lib/tollgate/types/ConnectedNetworkInfo";
import type {NetworkInfo} from "$lib/tollgate/types/NetworkInfo";
import {invoke} from "@tauri-apps/api/core";
import {fetch} from "@tauri-apps/plugin-http";

export default class AndroidOperatingSystem implements IOperatingSystem {

    // Depending on the OS we can or cannot get the mac from our device, in case of android we have to call the whoami service.
    // Ideally this happens inside the android plugin. Due to time constraints I've done it in two steps now. The 'androidwifi|getMacAddress' currently
    // only rebinds the network to the wifi interface, sho that our actual web request will go to the router and not use our cellular connection.
    async getMacAddress(gatewayIp: string | undefined): Promise<string | undefined> {
        if(gatewayIp === undefined){
            return undefined;
        }

        // Step 1, rebind the app to the wifi network, so the whoami request won't be sent over any active cellular connection
        try{
            await invoke("plugin:androidwifi|get_mac_address", { }); // only rebinds
        } catch (e) {
            console.error("could not get mac native", e);
        }

        // Step 2, call the whoami service
        const whoamiResponse = await fetch(`http://${gatewayIp}:2122/`, {connectTimeout: 350}).catch((reason) => {
            throw new Error(`Failed to determine MAC address, reason: ${reason}`);
        }) // Universal endpoint for whoami

        let whoami = await whoamiResponse.json();

        if(whoami.Success === false) {
            console.error(`Failed to determine MAC address, reason: ${whoami.ErrorMessage}`)
            return undefined;
        }

        return whoami.Mac
    }

    async getCurrentNetwork(): Promise<ConnectedNetworkInfo> {
        const currentNetworkInfo = await invoke("plugin:androidwifi|get_current_wifi_details", { })
        const details = JSON.parse(currentNetworkInfo.wifiDetails)

        const ssid = details.ssid.replaceAll('"',''); // TODO: bug in serialization from android

        console.log(`Current network: ${ssid}`);
        return {
            ssid: ssid,
        }
    }

    async getAvailableNetworks(): Promise<NetworkInfo[]> {
        let response = await invoke("plugin:androidwifi|get_wifi_details", { payload: { value: "" } });
        const networks: NetworkInfo[] = JSON.parse(response.wifis);

        return networks;
    }

    async connectNetwork(ssid: string): Promise<void> {
        let response = await invoke("plugin:androidwifi|connect_wifi", { ssid: ssid });
        console.log("response for connecting to " + ssid + " = " + JSON.stringify(response));
    }
}
