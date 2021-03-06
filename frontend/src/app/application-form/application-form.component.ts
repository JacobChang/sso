import { Component, OnInit, Output, EventEmitter } from "@angular/core";
import {
  FormBuilder,
  FormGroup,
  FormControl,
  Validators
} from "@angular/forms";
import { Router, ActivatedRoute } from "@angular/router";
import {
  Application,
  ApplicationModelService
} from "../application-model.service";
import { session } from "../model";

@Component({
  selector: "application-form",
  templateUrl: "./application-form.component.html",
  styleUrls: ["./application-form.component.css"]
})
export class ApplicationFormComponent implements OnInit {
  application: FormGroup;
  @Output() succeed = new EventEmitter();
  @Output() failed = new EventEmitter();

  constructor(
    private fb: FormBuilder,
    private applicationModelService: ApplicationModelService,
    private router: Router
  ) {
    this.application = fb.group({
      name: ["", [Validators.required]],
      website_uri: ["", [Validators.required]],
      callback_uri: ["", [Validators.required]]
    });
  }

  ngOnInit() {}

  create({ value, valid }: { value: Application; valid: boolean }) {
    this.applicationModelService.create(value).subscribe(
      (application: Application) => {
        this.succeed.emit(application);
      },
      error => {
        this.failed.emit(error);
      }
    );
  }
}
