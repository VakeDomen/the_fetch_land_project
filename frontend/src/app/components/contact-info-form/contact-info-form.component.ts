import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';


@Component({
  selector: 'app-contact-info-form',
  templateUrl: './contact-info-form.component.html',
  styleUrls: ['./contact-info-form.component.sass']
})
export class ContactInfoFormComponent implements OnInit {  

  @Output() private username = new EventEmitter<string>();
  @Output() private phone = new EventEmitter<string>();

  @Input() public usernameInput: string = "";
  @Input() public phoneInput: string = "";


  constructor() { }

  ngOnInit(): void {
  }

  emitChanges() {
    this.username.emit(this.usernameInput);
    this.phone.emit(this.phoneInput);
  }

}
