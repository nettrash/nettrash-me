import { Component, Inject, ViewChild } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { MatInput } from '@angular/material/input';

interface Luhn {
  result: boolean;
  luhnResult: boolean;
  errorText: string;
}

@Component({
  selector: 'math-luhn',
  templateUrl: './luhn.component.html',
  styleUrls: ['./luhn.component.css']
})
export class LuhnComponent {

  private httpClient: HttpClient;
  private requestUrl: string;

  public luhnResult: Luhn;

  @ViewChild(MatInput, { static: false }) source: MatInput;

  constructor(http: HttpClient) {
    this.httpClient = http;
    this.requestUrl = document.getElementsByTagName('base')[0].href + "math/luhn";
    this.luhnResult = { result: true, luhnResult: true, errorText: "" };
  }

  doCheckLuhn() {
    let params = new HttpParams();
    params = params.append('source', this.source.value);
    this.httpClient.get<Luhn>(this.requestUrl, { params: params }).subscribe(result => {
      this.luhnResult = result;
      this.source.errorState = !(this.luhnResult.result && this.luhnResult.luhnResult);
    }, error => console.error(error));
  }

  doEnterAction() {
    this.doCheckLuhn();
  }

  doChangeAction() {
    this.doCheckLuhn();
  }

  clearResult() {
    this.source.value = "";
    this.luhnResult = { result: true, luhnResult: true, errorText: "" };
    this.source.errorState = false;
  }
}
