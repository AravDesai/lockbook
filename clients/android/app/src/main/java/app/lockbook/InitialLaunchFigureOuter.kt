package app.lockbook

import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import android.os.Bundle
import android.util.Log
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.biometric.BiometricConstants
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.preference.PreferenceManager
import app.lockbook.core.loadLockbookCore
import app.lockbook.loggedin.listfiles.ListFilesActivity
import app.lockbook.login.WelcomeActivity
import app.lockbook.utils.SharedPreferences.BIOMETRIC_NONE
import app.lockbook.utils.SharedPreferences.BIOMETRIC_OPTION_KEY
import app.lockbook.utils.SharedPreferences.BIOMETRIC_RECOMMENDED
import app.lockbook.utils.SharedPreferences.BIOMETRIC_STRICT
import app.lockbook.utils.SharedPreferences.LOGGED_IN_KEY

class InitialLaunchFigureOuter : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.splash_screen)
        loadLockbookCore()

        val pref = PreferenceManager.getDefaultSharedPreferences(this)

        if (pref.getBoolean(LOGGED_IN_KEY, false)) {
            if (!isBiometricsOptionsAvailable() && pref.getString(
                    BIOMETRIC_OPTION_KEY,
                    BIOMETRIC_NONE
                ) != BIOMETRIC_NONE
            ) {
                pref.edit()
                    .putString(BIOMETRIC_OPTION_KEY, BIOMETRIC_NONE)
                    .apply()
            }
            performBiometricFlow(pref)
        } else {
            startActivity(Intent(this, WelcomeActivity::class.java))
            finish()
        }
    }

    private fun isBiometricsOptionsAvailable(): Boolean =
        BiometricManager.from(applicationContext)
            .canAuthenticate() == BiometricManager.BIOMETRIC_SUCCESS

    private fun performBiometricFlow(pref: SharedPreferences) {
        when (
            pref.getString(
                BIOMETRIC_OPTION_KEY, BIOMETRIC_NONE
            )
            ) {
            BIOMETRIC_STRICT -> {
                if (BiometricManager.from(applicationContext)
                        .canAuthenticate() != BiometricManager.BIOMETRIC_SUCCESS
                ) {
                    Toast.makeText(this, "An unexpected error has occurred!", Toast.LENGTH_LONG)
                        .show()
                    finish()
                }

                val executor = ContextCompat.getMainExecutor(this)
                val biometricPrompt = BiometricPrompt(
                    this, executor,
                    object : BiometricPrompt.AuthenticationCallback() {
                        override fun onAuthenticationError(
                            errorCode: Int,
                            errString: CharSequence
                        ) {
                            super.onAuthenticationError(errorCode, errString)
                            when (errorCode) {
                                BiometricConstants.ERROR_HW_UNAVAILABLE, BiometricConstants.ERROR_UNABLE_TO_PROCESS, BiometricConstants.ERROR_NO_BIOMETRICS, BiometricConstants.ERROR_HW_NOT_PRESENT -> {
                                    Log.i("Launch", "Biometric authentication error: $errString")
                                    Toast.makeText(
                                        applicationContext,
                                        "An unexpected error has occurred!", Toast.LENGTH_SHORT
                                    )
                                        .show()
                                    finish()
                                }
                                BiometricConstants.ERROR_LOCKOUT, BiometricConstants.ERROR_LOCKOUT_PERMANENT ->
                                    Toast.makeText(
                                        applicationContext,
                                        "Too many tries, try again later!", Toast.LENGTH_SHORT
                                    )
                                        .show()
                                else -> finish()
                            }
                        }

                        override fun onAuthenticationSucceeded(
                            result: BiometricPrompt.AuthenticationResult
                        ) {
                            super.onAuthenticationSucceeded(result)
                            startActivity(
                                Intent(
                                    applicationContext,
                                    ListFilesActivity::class.java
                                )
                            )
                            finish()
                        }
                    }
                )

                val promptInfo = BiometricPrompt.PromptInfo.Builder()
                    .setTitle("Lockbook Biometric Verification")
                    .setSubtitle("Enter your fingerprint to access lockbook.")
                    .setDeviceCredentialAllowed(true)
                    .build()

                biometricPrompt.authenticate(promptInfo)

            }
            BIOMETRIC_NONE, BIOMETRIC_RECOMMENDED -> {
                startActivity(Intent(applicationContext, ListFilesActivity::class.java))
                finish()
            }
        }
    }
}
