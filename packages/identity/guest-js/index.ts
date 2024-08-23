import {invoke} from '@tauri-apps/api/core';

import type {
  CreateIdentityVerificationSheetOption,
  IdentityVerificationSheetResultInterface,
  InitializeIdentityVerificationSheetOption
} from './definitions';


export async function initialize(options: InitializeIdentityVerificationSheetOption): Promise<void> {
  return await invoke<void>('plugin:stripe-identity|initialize', {
    payload: options,
  });
}

export async function create(options: CreateIdentityVerificationSheetOption): Promise<void> {
  return await invoke<void>('plugin:stripe-identity|create', {
    payload: options,
  });
}

export async function present(): Promise<{ identityVerificationResult: IdentityVerificationSheetResultInterface; }> {
  return await invoke<{ identityVerificationResult: IdentityVerificationSheetResultInterface; }>('plugin:stripe-identity|present');
}
