import { Component, Inject, ViewChild } from '@angular/core';
import { HttpClient, HttpParams, HttpHeaders } from '@angular/common/http';
import { MatInput } from '@angular/material/input';

interface Base64Value {
  result: boolean;
  value: string;
}

@Component({
  selector: 'text-base64',
  templateUrl: './base64.component.html',
  styleUrls: ['./base64.component.css']
})
export class Base64Component {

  private httpClient: HttpClient;
  private requestUrl: string;

  public base64Source: string;
  public base64Result: Base64Value;

  @ViewChild('sourceData', { static: false }) sourceData: MatInput;
  @ViewChild('base64Value', { static: false }) base64Value: MatInput;

  constructor(http: HttpClient) {
    this.httpClient = http;
    this.requestUrl = document.getElementsByTagName('base')[0].href + "text/base64";
    this.base64Result = { result: true, value: "" };
  }

  doBase64(encode: boolean) {
    let header = new HttpHeaders({ 'Content-Type': 'application/json' });
    let params = { encode: encode, source: this.base64Source };
    this.httpClient.post<Base64Value>(this.requestUrl, JSON.stringify(params), { headers: header }).subscribe(result => {
      this.base64Result = result;
      this.base64Value.value = this.base64Result.value;
    }, error => console.error(error));
  }

  doBase64Encode() {
    this.doBase64(true);
  }

  doBase64Decode() {
    this.doBase64(false);
  }

  clearResult() {
    this.base64Result = { result: true, value: "" };
    this.base64Source = "";
    this.base64Value.value = "";
  }
}
