import { animate, style, transition, trigger } from '@angular/animations';
import { Component, Input, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'app-privacy-policy',
  templateUrl: './privacy-policy.component.html',
  styleUrls: ['./privacy-policy.component.sass'],
  animations: [
    trigger('enter', [
       transition(':enter', [
        style({ transform: 'translateX(200%)' }),
        animate('200ms'),
      ])
    ]),
  ]  
})
export class PrivacyPolicyComponent implements OnInit {

  @Input() private changeTitle: boolean = false;

  constructor(
    private title: Title,
  ) { }

  ngOnInit(): void {
    this.title.setTitle("Varovanje osebnih podatkov | TheFethclandProject");
  }

}
