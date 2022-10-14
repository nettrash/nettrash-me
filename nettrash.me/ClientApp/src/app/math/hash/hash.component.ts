import { Component, Inject, ViewChild } from '@angular/core';
import { HttpClient, HttpParams, HttpHeaders } from '@angular/common/http';
import { MatInput } from '@angular/material/input';
import { MatSelect } from '@angular/material/select';

interface Algorithm {
  value: string;
  viewValue: string;
}

interface HashValue {
  result: boolean;
  value: string;
}

@Component({
  selector: 'math-hash',
  templateUrl: './hash.component.html',
  styleUrls: ['./hash.component.css']
})
export class HashComponent {

  private httpClient: HttpClient;
  private requestUrl: string;

  public algorithms: Algorithm[] = [
    { value: 'md5', viewValue: 'MD5' },
    { value: 'sha1', viewValue: 'SHA1' },
    { value: 'sha256', viewValue: 'SHA256' },
    { value: 'sha384', viewValue: 'SHA384' },
    { value: 'sha512', viewValue: 'SHA512' },
  ];
  public selectedAlgorithm = this.algorithms[0].value;
  public hashSource: string;
  public hashResult: HashValue;

  @ViewChild('algorithm', { static: false }) algorithm: MatSelect;
  @ViewChild('sourceText', { static: false }) sourceText: MatInput;
  @ViewChild('hashValue', { static: false }) hashValue: MatInput;

  constructor(http: HttpClient) {
    this.httpClient = http;
    this.requestUrl = document.getElementsByTagName('base')[0].href + "math/hash";
    this.hashResult = { result: true, value: "" };
  }

  doCalculateHash() {
    let header = new HttpHeaders({ 'Content-Type': 'application/json' });
    let params = { algorithm: this.algorithm.value, sourceText: this.hashSource };
    this.httpClient.post<HashValue>(this.requestUrl, JSON.stringify(params), { headers: header }).subscribe(result => {
      this.hashResult = result;
      this.hashValue.value = this.hashResult.value;
    }, error => console.error(error));
  }

  doEnterAction() {
    this.doCalculateHash();
  }

  doChangeAction() {
    this.doCalculateHash();
  }

  doAlgorithmChange() {
    this.doCalculateHash();
  }

  clearResult() {
    this.hashResult = { result: true, value: "" };
    this.hashSource = "";
    this.hashValue.value = "";
  }
}
