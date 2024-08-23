export * from './events.enum';
export interface StripeIdentityError {
  message: string;
}

export interface InitializeIdentityVerificationSheetOption {
  publishableKey: string;
}

export interface CreateIdentityVerificationSheetOption {
  verificationId: string;
  ephemeralKeySecret: string;

  /**
   * This client secret is used only for the web platform.
   */
  clientSecret?: string;
}
