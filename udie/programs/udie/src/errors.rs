use anchor_lang::error_code;

#[error_code]
pub enum UdieError {
    #[msg("Invalid Share Percentage")]
    InvalidSharePercentage,
    
    #[msg("Invalid Owner")]
    InvalidOwner,
    
    #[msg("Plan is Locked")]
    PlanLocked,
    
    #[msg("Death Not Verified")]
    DeathNotVerified,
    
    #[msg("Total Share Exceeds 100%")]
    ShareExceeds100Percent,
    
    #[msg("Invalid Relationship")]
    InvalidRelationship,
    
    #[msg("Invalid Amount")]
    InvalidAmount,
    
    #[msg("Overflow")]
    Overflow,
    
    #[msg("Division by Zero")]
    DivisionByZero,
    
    #[msg("Safety Period Active")]
    SafetyPeriodActive,
    
    #[msg("Freeze Period Active")]
    FreezePeriodActive,
    
    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Beneficiary Withdrawn")]
    BeneficiaryWithdrawn,

    #[msg("Invalid Beneficiary")]
    InvalidBeneficiary,
}
