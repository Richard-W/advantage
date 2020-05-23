#ifndef _ADV_FFI_HPP
#define _ADV_FFI_HPP

extern "C"
{

typedef struct adv_acontext adv_acontext;
typedef struct adv_adouble adv_adouble;

void adv_acontext_free(adv_acontext* self);
adv_acontext* adv_acontext_new(void);

adv_adouble* adv_acontext_new_independent(adv_acontext* self);
void adv_acontext_set_dependent(adv_acontext* self, const adv_adouble* val);

void adv_adouble_free(adv_adouble* self);
adv_adouble* adv_adouble_default(void);
adv_adouble* adv_adouble_copy(const adv_adouble* this_);
adv_adouble* adv_adouble_from_value(double val);

void adv_op_add(const adv_adouble* a, const adv_adouble* b, adv_adouble** result);
void adv_op_sub(const adv_adouble* a, const adv_adouble* b, adv_adouble** result);
void adv_op_mul(const adv_adouble* a, const adv_adouble* b, adv_adouble** result);
void adv_op_div(const adv_adouble* a, const adv_adouble* b, adv_adouble** result);

void adv_sin(const adv_adouble* x, adv_adouble** result);
void adv_cos(const adv_adouble* x, adv_adouble** result);
void adv_tan(const adv_adouble* x, adv_adouble** result);
void adv_abs(const adv_adouble* x, adv_adouble** result);
void adv_exp(const adv_adouble* x, adv_adouble** result);
void adv_ln(const adv_adouble* x, adv_adouble** result);

void adv_max(const adv_adouble* a, const adv_adouble* b, adv_adouble** result);
void adv_min(const adv_adouble* a, const adv_adouble* b, adv_adouble** result);

}

#endif // _ADV_FFI_HPP
