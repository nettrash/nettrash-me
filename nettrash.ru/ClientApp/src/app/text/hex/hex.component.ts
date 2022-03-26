import { Component, Inject, ViewChild } from '@angular/core';
import { HttpClient, HttpParams, HttpHeaders } from '@angular/common/http';
import { MatInput } from '@angular/material/input';

interface HexValue {
  result: boolean;
  value: string;
}

@Component({
  selector: 'text-hex',
  templateUrl: './hex.component.html',
  styleUrls: ['./hex.component.css']
})
export class HexComponent {

  private httpClient: HttpClient;
  private requestUrl: string;

  public hexSource: string;
  public hexResult: HexValue;

  @ViewChild('sourceData', { static: false }) sourceData: MatInput;
  @ViewChild('hexValue', { static: false }) hexValue: MatInput;

  constructor(http: HttpClient) {
    this.httpClient = http;
    this.requestUrl = document.getElementsByTagName('base')[0].href + "text/hex";
    this.hexResult = { result: true, value: "" };
  }

  doHex(encode: boolean) {
    let header = new HttpHeaders({ 'Content-Type': 'application/json' });
    let params = { encode: encode, source: this.hexSource };
    this.httpClient.post<HexValue>(this.requestUrl, JSON.stringify(params), { headers: header }).subscribe(result => {
      this.hexResult = result;
      this.hexValue.value = this.hexResult.value;
    }, error => console.error(error));
  }

  doHexEncode() {
    this.doHex(true);
  }

  doHexDecode() {
    this.doHex(false);
  }

  clearResult() {
    this.hexResult = { result: true, value: "" };
    this.hexSource = "";
    this.hexValue.value = "";
  }
}
