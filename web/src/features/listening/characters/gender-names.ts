import type { Gender } from '@/bindings'

export const genderNames: Record<Gender, string> = { female: '女', male: '男' }

/** The order the choices are offered in. */
export const genders: readonly Gender[] = ['female', 'male']
