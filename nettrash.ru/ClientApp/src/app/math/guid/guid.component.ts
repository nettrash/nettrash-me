import { Component, Inject, ViewChild } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { MatTable } from '@angular/material/table';


interface Guid {
  value: string;
}

@Component({
  selector: 'math-guid',
  templateUrl: './guid.component.html',
  styleUrls: ['./guid.component.css']
})
export class GuidComponent {

  @ViewChild(MatTable, { static: false }) table: MatTable<Guid>;

  private httpClient: HttpClient;
  private requestUrl: string;

  public displayedColumns: string[] = ['value'];
  public guids: Guid[];

  constructor(http: HttpClient) {
    this.httpClient = http;
    this.requestUrl = document.getElementsByTagName('base')[0].href + "math/guid";
    this.guids = [];
  }

  doGenerateGuid() {
    this.httpClient.get<Guid>(this.requestUrl).subscribe(result => {
      this.guids = [result].concat(this.guids);
      this.table.renderRows();
    }, error => console.error(error));
  }

  clearGuid() {
    this.guids = [];
    this.table.renderRows();
  }

}
