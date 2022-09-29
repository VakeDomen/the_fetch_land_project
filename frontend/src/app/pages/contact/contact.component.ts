import { animate, style, transition, trigger } from '@angular/animations';
import { Component, OnInit } from '@angular/core';
import { ToastrService } from 'ngx-toastr';
import { NotificationMessage } from 'src/app/models/notificationn.model';
import { AuthService } from 'src/app/services/auth.service';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-contact',
  templateUrl: './contact.component.html',
  styleUrls: ['./contact.component.sass'],
  animations: [
    trigger('enter', [
       transition(':enter', [
        style({ transform: 'translateX(200%)' }),
        animate('200ms'),
      ])
    ]),
  ]  
})
export class ContactComponent implements OnInit {

  public textareaContent: string = "";

  constructor(
    private auth: AuthService,
    private data: DataService,
    private toastr: ToastrService,
  ) { }

  ngOnInit(): void {
  }

  public sendMessage() {
    const message = {} as NotificationMessage;
    message.message = this.textareaContent;
    message.sender = this.auth.getUserEmail();
    this.data.sendNotification(message).subscribe((resp: any) => this.messageSuccess());
  }

  private messageSuccess() {
    this.toastr.success("Sporočilo uspešno poslano!", "")
    this.textareaContent = "";
  }

  public routeToLogin() {
    document.location = this.auth.getGoogleLoginUrl();
  }

  public notLoggedIn() {
    return !this.auth.isLoggedIn();
  }
}
