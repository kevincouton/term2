package com.term2

import android.app.Application

class Term2Application : Application() {
    companion object {
        init {
            System.loadLibrary("term2_android")
        }
    }
}
