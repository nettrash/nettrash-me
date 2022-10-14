import { Component, Inject, ViewChild } from '@angular/core';
import { HttpClient, HttpParams, HttpHeaders } from '@angular/common/http';
import { MatInput } from '@angular/material/input';

interface UrlValue {
  result: boolean;
  value: string;
}

@Component({
  selector: 'text-url',
  templateUrl: './url.component.html',
  styleUrls: ['./url.component.css']
})
export class UrlComponent {

  private httpClient: HttpClient;
  private requestUrl: string;

  public urlSource: string;
  public urlResult: UrlValue;

  @ViewChild('sourceData', { static: false }) sourceData: MatInput;
  @ViewChild('urlValue', { static: false }) urlValue: MatInput;

  constructor(http: HttpClient) {
    this.httpClient = http;
    this.requestUrl = document.getElementsByTagName('base')[0].href + "text/url";
    this.urlResult = { result: true, value: "" };
  }

  doUrl(encode: boolean) {
    let header = new HttpHeaders({ 'Content-Type': 'application/json' });
    let params = { encode: encode, source: this.urlSource };
    this.httpClient.post<UrlValue>(this.requestUrl, JSON.stringify(params), { headers: header }).subscribe(result => {
      this.urlResult = result;
      this.urlValue.value = this.urlResult.value;
    }, error => console.error(error));
  }

  doUrlEncode() {
    this.doUrl(true);
  }

  doUrlDecode() {
    this.doUrl(false);
  }

  clearResult() {
    this.urlResult = { result: true, value: "" };
    this.urlSource = "";
    this.urlValue.value = "";
  }
}
