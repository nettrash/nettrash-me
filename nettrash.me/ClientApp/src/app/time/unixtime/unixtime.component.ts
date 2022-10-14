import { Component, Inject, ViewChild } from '@angular/core';
import { HttpClient, HttpParams, HttpHeaders } from '@angular/common/http';
import { MatInput } from '@angular/material/input';
import { MatSelect } from '@angular/material/select';
import { encode } from 'punycode';

interface UnixtimeValue {
  result: boolean;
  value: string;
}

@Component({
  selector: 'time-unixtime',
  templateUrl: './unixtime.component.html',
  styleUrls: ['./unixtime.component.css']
})
export class UnixtimeComponent {

  private httpClient: HttpClient;
  private requestUrl: string;

  public result: UnixtimeValue;
  public sourceText: string;

  @ViewChild('sourceData', { static: false }) sourceData: MatInput;
  @ViewChild('resultValue', { static: false }) resultValue: MatInput;

  constructor(http: HttpClient) {
    this.httpClient = http;
    this.requestUrl = document.getElementsByTagName('base')[0].href + "time/unixtime";
    this.result = { result: true, value: "" };
  }

  doConvert() {
    let params = new HttpParams();
    params = params.append('source', encodeURIComponent(this.sourceText));
    this.httpClient.get<UnixtimeValue>(this.requestUrl, { params: params }).subscribe(result => {
      this.result = result;
    }, error => console.error(error));
  }

  doEnterAction() {
    this.doConvert();
  }

  clearResult() {
    this.result = { result: true, value: "" };
    this.sourceData.value = "";
    this.resultValue.value = "";
    this.sourceText = "";
  }
}
