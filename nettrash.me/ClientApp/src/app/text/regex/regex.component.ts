import { Component, Inject, ViewChild } from '@angular/core';
import { HttpClient, HttpParams, HttpHeaders } from '@angular/common/http';
import { MatInput } from '@angular/material/input';

interface RegExValue {
	result: boolean;
	value: string;
}

@Component({
	selector: 'text-regex',
	templateUrl: './regex.component.html',
	styleUrls: ['./regex.component.css']
})
export class RegExComponent {

	private httpClient: HttpClient;
	private requestUrl: string;

	public regexSource: string;
	public regexText: string;
	public regexResult: RegExValue;

	@ViewChild('sourceData', { static: false }) sourceData: MatInput;
	@ViewChild('textData', { static: false }) textData: MatInput;
	@ViewChild('regexValue', { static: false }) regexValue: MatInput;

	constructor(http: HttpClient) {
		this.httpClient = http;
		this.requestUrl = document.getElementsByTagName('base')[0].href + "text/regex";
		this.regexResult = { result: true, value: "" };
	}

	doRegexCheck() {
		let header = new HttpHeaders({ 'Content-Type': 'application/json' });
		let params = { source: this.regexSource, text: this.regexText };
		this.httpClient.post<RegExValue>(this.requestUrl, JSON.stringify(params), { headers: header }).subscribe(result => {
			this.regexResult = result;
			this.regexValue.value = this.regexResult.value;
		}, error => console.error(error));
	}

	clearResult() {
		this.regexResult = { result: true, value: "" };
		this.regexSource = "";
		this.regexText = "";
		this.regexValue.value = "";
	}
}
