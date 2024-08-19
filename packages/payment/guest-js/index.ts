import { invoke } from '@tauri-apps/api/core'

import type {
  ApplePayResultInterface,
  CreateApplePayOption,
  CreateGooglePayOption,
  CreatePaymentFlowOption,
  CreatePaymentSheetOption,
  GooglePayResultInterface,
  PaymentFlowResultInterface,
  PaymentSheetResultInterface,
  StripeInitializationOptions,
} from './definitions';

export async function initialize(options: StripeInitializationOptions): Promise<void> {
  return await invoke<void>('plugin:stripe-payment|initialize', {
    payload: options,
  });
}

export async function  createPaymentSheet(options: CreatePaymentSheetOption): Promise<void> {
  return await invoke<void>('plugin:stripe-payment|createPaymentSheet', {
    payload: options,
  });
}

export async function  presentPaymentSheet(): Promise<{
  paymentResult: PaymentSheetResultInterface;
}> {
  return await invoke<{
    paymentResult: PaymentSheetResultInterface;
  }>('plugin:stripe-payment|presentPaymentSheet', {
    payload: {},
  });
}

export async function createPaymentFlow(options: CreatePaymentFlowOption): Promise<void> {
  return await invoke<void>('plugin:stripe-payment|createPaymentFlow', {
    payload: options,
  });
}

export async function  presentPaymentFlow(): Promise<{
  cardNumber: string;
}> {
  return await invoke<{
    cardNumber: string;
  }>('plugin:stripe-payment|presentPaymentFlow', {
    payload: {},
  });
}

export async function  confirmPaymentFlow(): Promise<{
  paymentResult: PaymentFlowResultInterface;
}> {
  return await invoke<{
    paymentResult: PaymentFlowResultInterface;
  }>('plugin:stripe-payment|confirmPaymentFlow', {
    payload: {},
  });
}

export async function isApplePayAvailable(): Promise<void> {
  return await invoke<void>('plugin:stripe-payment|isApplePayAvailable', {
    payload: {},
  });
}

export async function createApplePay(createApplePayOption: CreateApplePayOption): Promise<void> {
  return await invoke<void>('plugin:stripe-payment|createApplePay', {
    payload: createApplePayOption,
  });
}

export async function presentApplePay(): Promise<{
  paymentResult: ApplePayResultInterface;
}> {
  return await invoke<{
    paymentResult: ApplePayResultInterface;
  }>('plugin:stripe-payment|presentApplePay', {
    payload: {},
  });
}

export async function isGooglePayAvailable(): Promise<void> {
  return await invoke<void>('plugin:stripe-payment|isGooglePayAvailable', {
    payload: {},
  });
}

export async function createGooglePay(createGooglePayOption: CreateGooglePayOption): Promise<void> {
  return await invoke<void>('plugin:stripe-payment|createGooglePay', {
    payload: createGooglePayOption,
  });
}

export async function presentGooglePay(): Promise<{
  paymentResult: GooglePayResultInterface;
}> {
  return await invoke<{
    paymentResult: GooglePayResultInterface;
  }>('plugin:stripe-payment|presentGooglePay', {
    payload: {},
  });
}
