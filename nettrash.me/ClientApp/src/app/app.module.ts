import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { HttpClientModule } from '@angular/common/http';
import { RouterModule } from '@angular/router';

import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

import { AgmCoreModule } from '@agm/core';

import { MatTabsModule } from '@angular/material/tabs';
import { MatButtonModule } from '@angular/material/button';
import { MatTableModule } from '@angular/material/table';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatIconModule } from '@angular/material/icon';
import { MatSelectModule } from '@angular/material/select';
import {MatCardModule} from '@angular/material/card';

import { AppComponent } from './app.component';
import { NavMenuComponent } from './nav-menu/nav-menu.component';
import { HomeComponent } from './home/home.component';
import { ClientInfoComponent } from './home/info/info.component';
import { MathComponent } from './math/math.component';
import { GuidComponent } from './math/guid/guid.component';
import { LuhnComponent } from './math/luhn/luhn.component';
import { HashComponent } from './math/hash/hash.component';
import { TextComponent } from './text/text.component';
import { TimeComponent } from './time/time.component';
import { UnixtimeComponent } from './time/unixtime/unixtime.component';
import { Base64Component } from './text/base64/base64.component';
import { UrlComponent } from './text/url/url.component';
import { HexComponent } from './text/hex/hex.component';

@NgModule({
  declarations: [
    AppComponent,
    NavMenuComponent,
    HomeComponent,
    ClientInfoComponent,
    MathComponent,
    GuidComponent,
    LuhnComponent,
    HashComponent,
    TextComponent,
    TimeComponent,
    UnixtimeComponent,
    Base64Component,
    UrlComponent,
    HexComponent
  ],
  imports: [
    BrowserModule.withServerTransition({ appId: 'ng-cli-universal' }),
    HttpClientModule,
    FormsModule,
    BrowserAnimationsModule,
    AgmCoreModule.forRoot({apiKey:'AIzaSyC0jvdham6G17Agi-BOnq0QAMH2NgCvepw'}),
    MatTabsModule,
    MatButtonModule,
    MatTableModule,
    MatInputModule,
    MatFormFieldModule,
    MatIconModule,
    MatSelectModule,
    MatCardModule,
    RouterModule.forRoot([
      { path: '', component: HomeComponent, pathMatch: 'full' },
      { path: 'math', component: MathComponent, pathMatch: 'full' },
      { path: 'text', component: TextComponent, pathMatch: 'full' },
      { path: 'time', component: TimeComponent, pathMatch: 'full' },
    ])
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
