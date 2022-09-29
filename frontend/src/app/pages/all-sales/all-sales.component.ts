import { animate, style, transition, trigger } from '@angular/animations';
import { Component, OnInit } from '@angular/core';

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

  constructor() { }

  ngOnInit(): void {
  }

}
