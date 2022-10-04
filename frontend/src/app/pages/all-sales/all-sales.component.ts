import { animate, style, transition, trigger } from '@angular/animations';
import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'app-all-sales',
  templateUrl: './all-sales.component.html',
  styleUrls: ['./all-sales.component.sass'],
  animations: [
    trigger('enter', [
       transition(':enter', [
        style({ transform: 'translateX(200%)' }),
        animate('200ms'),
      ])
    ]),
  ]  
})
export class AllSalesComponent implements OnInit {

  constructor(
    private title: Title,
  ) { }

  ngOnInit(): void {
    this.title.setTitle("Nove ponudbe | TheFetchlandProject")
  }

}
