import { Component, Inject, ViewChild } from '@angular/core';
import { HttpClient, HttpParams, HttpHeaders } from '@angular/common/http';
import { MatInput } from '@angular/material/input';
import { inherits } from 'util';

interface ClientInfo {
  result: boolean;
  ipAddress: string;
}

@Component({
  selector: 'client-info',
  templateUrl: './info.component.html',
  styleUrls: ['./info.component.css']
})
export class ClientInfoComponent {

  private httpClient: HttpClient;
  private requestUrl: string;

  public clientInfo: ClientInfo;
  public clientDate: string;
  public clientTime: string;
  public clientUTC: string;
  public clientLocation: string;

  public latitude: number;
  public longitude: number;

  @ViewChild('childIPAddress', { static: false }) ipAddress: MatInput;
  @ViewChild('childClientDate', { static: false }) date: MatInput;
  @ViewChild('childClientTime', { static: false }) time: MatInput;
  @ViewChild('childUTC', { static: false }) utc: MatInput;
  @ViewChild('childLocation', { static: false }) coordinates: MatInput;

  constructor(http: HttpClient) {
    this.httpClient = http;
    this.requestUrl = document.getElementsByTagName('base')[0].href + "home/info";
    this.clientInfo = { result: true, ipAddress: "" };

    this.getClientInfo();
    setInterval(() => { this.getClientTime(); }, 1);

    this.getLocation();
  }

  getClientInfo() {
    let header = new HttpHeaders({ 'Content-Type': 'application/json' });
    let params = {};
    this.httpClient.post<ClientInfo>(this.requestUrl, JSON.stringify(params), { headers: header }).subscribe(result => {
      this.clientInfo = result;
      this.ipAddress.value = result.ipAddress;
    }, error => console.error(error));
  }

  getClientTime() {
    var now = new Date();

    this.clientDate = now.toLocaleDateString();
    this.date.value = now.toLocaleTimeString();

    this.clientTime = now.toLocaleTimeString();
    this.time.value = now.toLocaleTimeString();

    this.clientUTC = now.toUTCString();
    this.utc.value = now.toUTCString();
  }

  getLocation() {
    var page = this;
    navigator.geolocation.getCurrentPosition(function (position) {
      console.log(position.coords);
      var val = new Array(position.coords.latitude.toString(), position.coords.longitude.toString()).join(", ");
      console.log(val);
      page.clientLocation = val;
      page.coordinates.value = val;
      page.latitude = position.coords.latitude;
      page.longitude = position.coords.longitude;
    }, function () {
      page.latitude = 0;
      page.longitude = 0;
      page.clientLocation = "Access to the Location service is not allowed.";
      page.coordinates.value = "Access to the Location service is not allowed.";
    }, { timeout: 10000 });
  }
}